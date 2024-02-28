-- This is purely for performance
CREATE INDEX depicted_people_post_id ON post_depicted_people (post_id);
CREATE INDEX likes_post_id ON post_likes (post_id);
CREATE INDEX downloads_post_id ON post_downloads (post_id);