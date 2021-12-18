| API               | TYPE   | DESC                   | Stat               |
| ----------------- | ------ | ---------------------- | ------------------ |
| `user`            | POST   | new user(bulk)         | :heavy_check_mark: |
| `user`            | DELETE | (bulk)                 |                    |
| `user`            | GET    | get users              | :heavy_check_mark: |
| `user/:id`        | GET    | details                |                    |
| `auth`            | POST   | Login                  |                    |
| `auth`            | DELETE | Logout                 |                    |
| `dept`            | GET    | list departments       | :heavy_check_mark: |
| `dept`            | POST   | Create dept            |                    |
| `dept`            | DELETE | Delete dept            |                    |
| `subject`         | POST   | Create subject (bulk)  | :heavy_check_mark: |
| `subject`         | GET    | Get user subject list  |                    |
| `subject/:sid`    | GET    | GET ONE subject        | :heavy_check_mark: |
| `subject/:sid`    | DELETE | Delete subject         |                    |
| `book`            | POST   | Create booking         |                    |
| `book`            | DELETE | Unbook                 |                    |
| `stat/:sem/:dept` | GET    | Make+get bookings list |                    |
