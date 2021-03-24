
use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
enum ResponseType{
    Error,
    NewMessage,
    UserList,
    NewUser,
    FetchedMessages,
    InstanceCreated,
    InstanceAccepted,

}
#[derive(Serialize, Deserialize)]
enum ErrorType{
    WrongKey
}

#[derive(Serialize, Deserialize)]
struct AppError{
    res_type: ResponseType,
}

// Status of last requested action. Like `Create Instance` or `LoadUsers`
struct ActionStatus{
    
}
