| API               | TYPE   | DESC                           | Stat               |
| ----------------- | ------ | ------------------------------ | ------------------ |
| `user`            | POST   | new user(bulk)                 |                    |
| `user`            | DELETE | (bulk)                         |                    |
| `user`            | GET    | get users                      | :heavy_check_mark: |
| `user/:id`        | GET    | details                        |                    |
| `auth`            | POST   | Login                          |                    |
| `auth`            | DELETE | Logout                         |                    |
| `dept`            | GET    | list departments               | :heavy_check_mark: |
| `dept`            | POST   | Create dept                    |                    |
| `dept`            | DELETE | Delete dept                    |                    |
| `subject`         | POST   | Create subject                 | :heavy_check_mark: |
| `subject`         | POST   | Create subject (bulk)          |                    |
| `subject`         | GET    | Get subject list for user      |                    |
| `subject/:sid`    | GET    | GET ONE subject                | :heavy_check_mark: |
| `subject/my`?     | GET    | **NO** get list filtered       |                    |
| `subject/:sid`    | DELETE | Delete subject                 |                    |
| `book`            | POST   | Create booking                 |                    |
| `book`            | DELETE | Unbook                         |                    |
| `stat/:sem/:dept` | GET    | Process & get list of bookings |                    |
