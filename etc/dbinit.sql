create table if not exists dept(
    deptid char(3) primary key,
    name varchar(64) not null
);

create table if not EXISTS userauth (
    uid serial primary key,
    email varchar(64) unique not null,
    name varchar(32) not null,
    pwd varchar(512) not null,
    deptid char(3) not null,
    semester int,
    role int not null,
    foreign key (deptid) references dept(deptid)
);

create table if not EXISTS course (
    coursecode char(8) primary key not null,
    name varchar(32) not null,
    semester int not null,
    isglobal boolean default false not null,
    deptid char(3) not null,
    maxcapacity int not null,
    foreign key (deptid) references dept(deptid),
    check (semester >= 0)
);

create table if not EXISTS booking (
    uid int not null,
    coursecode char(8) not null,
    insert_time timestamp with time zone not null default now(),
    allocated boolean default false,
    foreign key (uid) references userauth(uid),
    foreign key (coursecode) references course(coursecode)
);

begin;

-- default data start
insert into dept
values('CSE', 'Computer Science and Engineering'),
    ('CV', 'Civil Engineering'),
    ('ME', 'Mechanical Engineering'),
    ('ISE', 'Information Science Engineering'),
    ('PY', 'Physics');


-- WARNING: default passwords: `password`
insert into userauth values
(1,'admin@localhost','Admin','$argon2id$v=19$m=19456,t=2,p=1$QUxCa3NzYWRqYnZraXNlaGJmaTN1b2gybzE$Fz7zQLKZPavq58Dx1qPHhQnyX2oufIOHtqhgnF0W/hk',
'CSE',3,0);

insert into userauth values
(2,'student@localhost','Student','$argon2id$v=19$m=19456,t=2,p=1$QUxCa3NzYWRqYnZraXNlaGJmaTN1b2gybzE$Fz7zQLKZPavq58Dx1qPHhQnyX2oufIOHtqhgnF0W/hk',
'CSE',3,1);

commit;
