CREATE TYPE habit_kind AS ENUM ('habit', 'addiction');
CREATE TYPE recording_type AS ENUM ('point', 'span');
CREATE TYPE span_part AS ENUM ('start', 'end');

CREATE TABLE "habits" (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    color VARCHAR(6) NOT NULL,
    kind habit_kind NOT NULL,
    recording_type recording_type NOT NULL,
    every INTEGER CHECK(every > 0) 
);

CREATE TABLE "events" (
    id SERIAL PRIMARY KEY,
    habit_id INTEGER NOT NULL REFERENCES habits(id) ON DELETE CASCADE,
    time TIMESTAMP NOT NULL,
    span_part span_part
);

CREATE INDEX idx_events_time_activity_id ON events(time, habit_id);
CREATE INDEX idx_events_activity_id ON events(habit_id);
CREATE INDEX idx_activities_id ON habits(id);

