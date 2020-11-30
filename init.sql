CREATE TABLE todo (
    id serial,
    title varchar(255),
    checked boolean not null default false,
    created_at timestamp default CURRENT_TIMESTAMP,
    updated_at timestamp default null,
    deleted_at timestamp default null,
    primary key (id)
);

INSERT INTO todo (title) VALUES ('Todo 1'), ('Todo 2');