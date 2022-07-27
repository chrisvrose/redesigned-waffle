create table if not exists dept(
    deptid char(3) primary key,
    name varchar(64) not null
);
create table if not EXISTS userauth (
    uid serial primary key,
    email varchar(64) unique not null,
    name varchar(32) not null,
    pwd varchar(512) not null,
    semester int not null,
    deptid char(3) not null,
    foreign key (deptid) references dept(deptid),
    check (semester >= 0)
);

create table if not exists teachers(
    uid serial primary key,
    email varchar(64) unique not null,
    name varchar(32) not null,
    pwd varchar(512) not null
);
create table if not EXISTS subject (
    coursecode char(8) primary key not null,
    name varchar(32) not null,
    semester int not null,
    isglobal boolean default false not null,
    deptid char(3) not null,
    maxcapacity int not null,
    foreign key (deptid) references dept(deptid),
    check (semester >= 0)
);

create table if not EXISTS book (
    uid int not null,
    coursecode char(8) not null,
    insert_time timestamp with time zone not null default now(),
    foreign key (uid) references userauth(uid),
    foreign key (coursecode) references subject(coursecode)
);
begin;
insert into dept
values('CSE', 'Computer Science and Engineering'),
    ('CV', 'Civil Engineering'),
    ('ME', 'Mechanical Engineering'),
    ('ISE', 'Information Science Engineering'),
    ('PY', 'Physics');


commit;