DROP INDEX IF EXISTS idx_events_time_activity_id;
DROP INDEX IF EXISTS idx_events_activity_id;
DROP INDEX IF EXISTS idx_activities_id;

DROP TABLE IF EXISTS "events";
DROP TABLE IF EXISTS "activities";

DROP TYPE IF EXISTS span_part;
DROP TYPE IF EXISTS recording_type;
DROP TYPE IF EXISTS activity_kind;
