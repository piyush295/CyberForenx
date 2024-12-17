# CyberForenx Academy Database Documentation

This README file provides an overview of the database setup and usage for the CyberForenx Academy project.

## Database Structure

The database is structured to support the backend application, including necessary tables and relationships. The initial schema is defined in the `migrations/init.sql` file.

## Migrations

- **File:** `migrations/init.sql`
- **Description:** This file contains SQL commands to create the initial database schema. It should be executed to set up the database before running the application.

## Seed Data

- **File:** `seeds/seed.sql`
- **Description:** This file contains SQL commands to populate the database with initial data. It can be executed after the migrations to ensure the database has the necessary starting data for development and testing.

## Usage

1. **Set up the database:**
   - Execute the SQL commands in `migrations/init.sql` to create the database schema.
   
2. **Seed the database:**
   - Run the SQL commands in `seeds/seed.sql` to populate the database with initial data.

3. **Connect to the database:**
   - Ensure that the backend application is configured to connect to the database correctly. Update the connection settings in the backend configuration files as needed.

## Additional Information

For more details on the backend application and how it interacts with the database, please refer to the backend documentation in `backend/README.md`.