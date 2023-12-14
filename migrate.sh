#!/usr/bin/env bash

# Function to create a migration file
create_migration() {
    # docker exec -it blog cargo sqlx prepare
    # docker exec -it blog 
    sqlx migrate add -r $1
}

# Function to run the migrations
run_migrations() {
    # docker exec -it blog 
    sqlx migrate run
}

# Function to run the down migrations
run_down_migrations() {
    # docker exec -it blog 
    sqlx migrate revert
}

# Check the command-line arguments and execute the corresponding functions
case $1 in
    "create")
        create_migration $2
        ;;
    "up")
        run_migrations
        ;;
    "down")
        run_down_migrations
        ;;
    *)
        echo "Invalid command. Usage: ./migrate.sh create <migration_name> | ./migrate.sh up | ./migrate.sh down"
        ;;
esac