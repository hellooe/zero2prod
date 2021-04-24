CREATE TABLE subscriptions(
  id uuid NOT NULL,
  email TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  subs_at timestamptz NOT NULL,
  PRIMARY KEY (id)
);
