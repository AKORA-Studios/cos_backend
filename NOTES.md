## Useful sources:

- https://codereview.stackexchange.com/questions/153710/sql-database-for-a-social-network

## TODO

Search for !TODO


POST /api/login
POST /api/register

GET  /api/users/<user_id>
GET  /api/users/<user_id>/posts?<limit>
GET  /api/users/<user_id>/messages?<limit>
POST /api/users/<to_user_id>/messages/new

POST /api/posts/new
GET  /api/posts/today?<limit>
GET  /api/posts/recent?<limit>
GET  /api/posts/<post_id>
POST /api/posts/<post_id>/like
POST /api/posts/<post_id>/dislike
POST /api/posts/<post_id>/download
POST /api/posts/<post_id>/comments/new
GET  /api/posts/<post_id>/comments/recent?<limit>

(
GET  /api/events/<event_id>
POST /api/events/new
)