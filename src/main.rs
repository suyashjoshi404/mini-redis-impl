use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    //creating a new TcpListener and binding it to a given addr
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        //creating an item containg ip and port of connection
        let (socket, _) = listener.accept().await.unwrap();

        //spawning a new task for each inbound socket
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    //creating a new empty hashmap
    let mut db = HashMap::new();

    //This will enable us to read/write redis frames
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                //Inserting key value pairs in the hashmap
                // The value is stored as `Vec<u8>`
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` expects data to be of type `Bytes`. This
                    // type will be covered later in the tutorial. For now,
                    // `&Vec<u8>` is converted to `Bytes` using `into()`.
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        //responsding to the client
        connection.write_frame(&response).await.unwrap();
    }
}
