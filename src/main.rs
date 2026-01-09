use gram::{parser::parse, tokenizer::tokenize};

fn main() {
    let sample_input = "\
Client -> Server: Login(username, password)
Server -> Database: ValidateCredentials()
Server <- Database: UserData
Server -> Cache: StoreSession()
Client <- Server: LoginSuccess(token)
Client -> Server: GetUserProfile(token)
Server -> Cache: CheckSession(token)
Server <- Cache: SessionValid
Server -> Database: FetchProfile(userId)
Server <- Database: ProfileData
Client <- Server: ProfileResponse(data)
Client -> Server: UpdateProfile(newData)
Server -> Database: UpdateRecord(userId, newData)
Server <- Database: UpdateConfirmed
Server -> Cache: InvalidateCache(userId)
Client <- Server: UpdateSuccess()
Client -> MessageQueue: PublishEvent(profileUpdated)
MessageQueue -> NotificationService: ProfileUpdatedEvent
NotificationService -> Client: PushNotification(changes)
";
    let tokens = tokenize(sample_input);
    if let Ok(tokens) = tokens {
        let graph = parse(tokens);

        if let Ok(graph) = graph {
            println!("{:#?}", graph);
        }
    }
}
