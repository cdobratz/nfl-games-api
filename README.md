# NFL Games API

A Rust-based API that provides NFL game schedules using The Rundown API. Built with Axum and deployed on AWS Lambda.

## API Endpoints

### GET /schedule
Returns the NFL game schedule.

**Response Format:**
```json
{
    "events": [
        {
            "event_id": "string",
            "sport_id": 2,
            "event_date": "string",
            "teams": {
                "home": {
                    "team_id": 1,
                    "name": "string"
                },
                "away": {
                    "team_id": 2,
                    "name": "string"
                }
            },
            "lines": {
                "spread": {
                    "point_spread_home": 0.0,
                    "point_spread_away": 0.0
                }
            }
        }
    ]
}
```

## Prerequisites

1. Rust toolchain (install via [rustup](https://rustup.rs/))
2. An API key from [The Rundown](https://the-odds-api.com/)
3. AWS account with Lambda access

## Local Development

1. Clone the repository:
   ```bash
   git clone https://github.com/cdobratz/nfl-games-api.git
   cd nfl-games-api
   ```

2. Copy .env.example to .env and add your API key:
   ```bash
   cp .env.example .env
   # Edit .env and add your RUNDOWN_API_KEY
   ```

3. Build and run locally:
   ```bash
   cargo build
   cargo run
   ```

## AWS Lambda Deployment

1. Build for Lambda:
   ```bash
   cargo build --release
   cd target/release
   cp nfl-games-api bootstrap
   zip lambda.zip bootstrap
   ```

2. Create a new Lambda function:
   - Go to AWS Lambda Console
   - Click 'Create function'
   - Choose 'Author from scratch'
   - Name: nfl-games-api
   - Runtime: Custom runtime on Amazon Linux 2
   - Architecture: x86_64

3. Configure the function:
   - Upload the lambda.zip file
   - Set environment variable RUNDOWN_API_KEY
   - Memory: 128 MB
   - Timeout: 30 seconds
   - Handler: leave as default

4. Set up function URL:
   - Click Configuration tab
   - Click Function URL
   - Auth type: NONE (for demo)
   - Configure CORS if needed

5. Test the deployment:
   - Copy the function URL
   - Test with: `<function-url>/schedule`

## Environment Variables

- `RUNDOWN_API_KEY`: Your API key from The Rundown

## Built With

- [Rust](https://www.rust-lang.org/)
- [Axum](https://github.com/tokio-rs/axum)
- [AWS Lambda](https://aws.amazon.com/lambda/)
- [The Rundown API](https://the-odds-api.com/)

## License

This project is licensed under the MIT License.

