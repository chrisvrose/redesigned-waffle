| API               | TYPE | DESC                   | Stat               | Auth |
| ----------------- | ---- | ---------------------- | ------------------ | ---- |
| `user`            | POST | new user(bulk)         | :heavy_check_mark: |      |
| `user`            | DEL  | (bulk)                 |                    |      |
| `user`            | GET  | get users              | :heavy_check_mark: |      |
| `user/:id`        | GET  | details                |                    |      |
| `auth`            | POST | Login                  | :heavy_check_mark: |   Y   |
| `auth/admin`      | POST | Login admin            |                    |      |
| `dept`            | GET  | list departments       | :heavy_check_mark: |      |
| `dept`            | POST | Create dept            |                    |      |
| `dept`            | DEL  | Delete dept            |                    |      |
| `subject`         | POST | Create subject (bulk)  | :heavy_check_mark: |      |
| `subject`         | GET  | Get user subject list  | :heavy_check_mark: | Y    |
| `subject/all`     | GET  | Get all subject list   | :heavy_check_mark: | NA   |
| `subject/:sid`    | GET  | GET ONE subject        | :heavy_check_mark: | NA   |
| `subject/:sid`    | DEL  | Delete subject         |                    |      |
| `book`            | POST | Create booking         |                    |      |
| `book`            | DEL  | Unbook                 |                    |      |
| `stat/:sem/:dept` | GET  | Make+get bookings list |                    |      |
