# random-rush
board game implementation project together with colleagues

# app id
app-id: 69b7ddce-882d-48d3-92f2-da3873a14304 <br />
use this ID during testing when you have to create any external resources

# how to run all together
docker build -t random-rush-dev . &&
docker run -p 3080:3000 -d --rm --name random-rush-dev random-rush-dev

open http://localhost:3080/


docker stop random-rush-dev

# how to start server
cargo run <br />
open http://localhost:3000/ in browser

# how to generate bindings
cargo test
