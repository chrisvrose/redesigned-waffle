| API                    | TYPE   | DESC                                   | Stat               |
| ---------------------- | ------ | -------------------------------------- | ------------------ |
| `user`                 | POST   | new user(bulk)                         |                    |
| `user`                 | DELETE | (bulk)                                 |                    |
| `user`                 | GET    | get users                              | :heavy_check_mark: |
| `user/:id`             | GET    | details                                |                    |
| `auth`                 | POST   | Login                                  |                    |
| `auth`                 | DELETE | Logout                                 |                    |
| `dept`                 | GET    | list departments                       | :heavy_check_mark: |
| `dept`                 | POST   | Create dept                            |                    |
| `dept`                 | DELETE | Delete dept                            |                    |
| `subject`              | POST   | Create subject                         |                    |
| `subject`              | GET    | Get subject list for user              |                    |
| `subject/my`?          | GET    | **NO** get list filtered               |                    |
| `subject/:sid`         | DELETE | Delete subject                         |                    |
| `book`                 | POST   | Create booking (with preference)       |                    |
| `book`                 | DELETE | Unbook                                 |                    |
| `formulate/:sem/:dept` | GET    | Process and get list of final bookings |                    |
