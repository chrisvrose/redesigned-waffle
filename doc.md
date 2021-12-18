| API               | TYPE | DESC                   | Stat               |
| ----------------- | ---- | ---------------------- | ------------------ |
| `user`            | POST | new user(bulk)         | :heavy_check_mark: |
| `user`            | DEL  | (bulk)                 |                    |
| `user`            | GET  | get users              | :heavy_check_mark: |
| `user/:id`        | GET  | details                |                    |
| `auth`            | POST | Login                  |                    |
| `dept`            | GET  | list departments       | :heavy_check_mark: |
| `dept`            | POST | Create dept            |                    |
| `dept`            | DEL  | Delete dept            |                    |
| `subject`         | POST | Create subject (bulk)  | :heavy_check_mark: |
| `subject`         | GET  | Get user subject list  |                    |
| `subject/all`     | GET  | Get all subject list   | :heavy_check_mark: |
| `subject/:sid`    | GET  | GET ONE subject        | :heavy_check_mark: |
| `subject/:sid`    | DEL  | Delete subject         |                    |
| `book`            | POST | Create booking         |                    |
| `book`            | DEL  | Unbook                 |                    |
| `stat/:sem/:dept` | GET  | Make+get bookings list |                    |
