mod router {
    trait Router {
        pub fn dispatch(env: call::EnvironmentMap, binargs: list<u8>) -> call::CallResult
    }
}
