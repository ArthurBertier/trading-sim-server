Trading Simulation Server Setup
This guide will walk you through setting up and running the Trading Simulation server, which includes connecting to a MongoDB database.

Prerequisites
Rust programming environment set up on your machine.
MongoDB installed and running on your machine or accessible remotely.
Git installed on your machine (for cloning the repository).
Getting Started
Step 1: Clone the Repository
Begin by cloning the repository to your local machine:

bash
Copy code
git clone https://github.com/ArthurBertier/trading-sim-server.git
cd trading-sim-server
Step 2: Install Dependencies
Ensure that you have Cargo installed (the Rust package manager), and run the following command to install necessary Rust dependencies:

bash
Copy code
cargo build --release
Step 3: Configure MongoDB Connection
Edit the MongoDB connection settings in your server's configuration file or environment variables. If the server's connection settings are not already configured, you might need to create a configuration file or pass environment variables.

Here’s a basic example of what your configuration might look like (typically in a .env or similar file):

plaintext
Copy code
MONGO_URI=mongodb://localhost:27017
MONGO_DB_NAME=trading_sim
Replace localhost:27017 with your MongoDB server's address and port. Change trading_sim to your specific database name.

Step 4: Running the Server
Once the configuration is set, you can run the server using Cargo:

bash
Copy code
cargo run --release
This command compiles the Rust project and runs the resulting binary.

Step 5: Verifying the Setup
Verify that your server is running correctly by accessing it in your web browser or using a tool like curl to hit the exposed endpoints. Check the logs for any MongoDB connection errors or success messages.

Changing Server Address
If your server's address needs to be adjusted (especially relevant if it doesn’t start with "20"), ensure your MongoDB host and server IP settings are correctly configured in your .env or configuration files.

Troubleshooting
If you encounter issues, check the following:

Ensure MongoDB is running and accessible from your server.
Check that all environment variables or configuration files are correctly set.
Make sure there are no network issues preventing your server from connecting to MongoDB.
Conclusion
Your Trading Simulation server should now be set up and connected to MongoDB. For more details on operations or additional configurations, refer to the specific documentation of the tools and libraries you are using.

