--- DELETE INVALID EVENT ENTRIES
CREATE OR REPLACE FUNCTION delete_events_when_type_changed()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.recording_type <> OLD.recording_type THEN
        DELETE FROM events
        WHERE habit_id = NEW.id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER delete_events_when_type_changed
AFTER UPDATE ON habits
FOR EACH ROW
EXECUTE FUNCTION delete_events_when_type_changed();

--- CHECK SPAn
CREATE OR REPLACE FUNCTION check_span_part()
RETURNS TRIGGER AS $$
DECLARE
    recording_type_value recording_type;
BEGIN
    -- Get the recording_type from the habits table
    SELECT h.recording_type INTO recording_type_value
    FROM habits h
    WHERE h.id = NEW.habit_id;

    -- Check the conditions
    IF recording_type_value = 'point' THEN
        IF NEW.span_part IS NOT NULL THEN
            RAISE EXCEPTION 'span_part must be NULL when recording_type is point';
        END IF;
    ELSE
        IF NEW.span_part IS NULL THEN
            RAISE EXCEPTION 'span_part must not be NULL when recording_type is not point';
        END IF;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER check_span_part_trigger
BEFORE INSERT OR UPDATE ON events
FOR EACH ROW EXECUTE FUNCTION check_span_part();
