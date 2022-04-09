CREATE TABLE users (
	id UUID PRIMARY KEY,
	username VARCHAR NOT NULL,
	hashed_password VARCHAR NOT NULL,
	created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

create index users_username_idx on users (username);

create table auth_tokens (
	id uuid primary key,
	user_id uuid not null references users(id),
	token varchar not null
);
