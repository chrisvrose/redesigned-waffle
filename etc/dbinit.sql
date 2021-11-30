create table if not EXISTS userauth (
    uid serial primary key,
    name varchar(32) not null,
    pwd varchar(64) not null,
    "semester" int not null,
    "dept" char(32) not null
);


create table if not EXISTS "subject" (
    coursecode char(8) primary key not null,
    "name" varchar(32) not null,
    semester  int not null,
    dept char(32) not null,
    maxcapacity  int not null
);

