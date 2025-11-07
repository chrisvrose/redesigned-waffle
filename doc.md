## User flows

### Auth

1. Admin registers through username, password.
2. User signs in using username password, gets token.
3. User uses token for getting profile details.

### Course creation

0. Admin views all course list.
1. Admin sets up course list.
2. Admin views their course list.


## API list

| API               | TYPE | DESC                              | Stat               | Auth |
| ----------------- | ---- | --------------------------------- | ------------------ | ---- |
| `user`            | POST | new user                          | :heavy_check_mark: |      |
| `user`            | DEL  | (bulk)                            |                    |      |
| `user`            | GET  | get users                         | :heavy_check_mark: |      |
| `user/:id`        | GET  | details                           |                    |      |
| `auth`            | POST | Login                             | :heavy_check_mark: | Y    |
| `auth/admin`      | POST | Login admin                       |                    |      |
| `dept`            | GET  | list departments                  | :heavy_check_mark: |      |
| `dept`            | POST | Create dept                       |                    |      |
| `dept`            | DEL  | Delete dept                       |                    |      |
| `subject`         | POST | Create subject (bulk)             | :heavy_check_mark: |      |
| `subject`         | GET  | Get user subject list             | :heavy_check_mark: | Y    |
| `subject/all`     | GET  | Get all subject list              | :heavy_check_mark: | NA   |
| `subject/:sid`    | GET  | GET ONE subject                   | :heavy_check_mark: | NA   |
| `subject/:sid`    | DEL  | Delete subject                    |                    |      |
| `book`            | POST | Create booking                    |                    |      |
| `book`            | GET  | GET booking for self              |                    |      |
| `book/:id`        | GET  | GET booking for user (admin only) |                    |      |
| `book`            | DEL  | Unbook                            |                    |      |
| `stat/:sem/:dept` | GET  | Make+get bookings list            |                    |      |



### Auth

Every route passes through [middleware.rs](src/misc/middleware.rs), which pulls out the extension header and gets its value.

Now, each route can configure its auth independently, by Requesting for `Option<web::ReqData<UserType>>`. If present, it is authenticated and the deets can be fetched. Else, unauthenticated.
