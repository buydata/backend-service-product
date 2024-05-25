model
  schema 1.1

type user

type subscribes
  relations
    define read: [user with non_expired_grant]
    
type data_product
  relations
    define subscribes: [subscribes]
    define read: read from subscribes
    define write: [user]

condition non_expired_grant(current_time: timestamp, grant_time: timestamp, grant_duration: duration) {
  current_time < grant_time + grant_duration
}