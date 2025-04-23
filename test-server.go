package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
)

type MigrationFileInfo struct {
	Id   string `json:"id"`
	Name string `json:"name"`
	URL  string `json:"url"`
}

func handleGetFiles(w http.ResponseWriter, r *http.Request) {
	fmt.Println("Request ", r)
	migrations := []MigrationFileInfo{
		{Id: "001", Name: "Initial schema", URL: "migrations/001_initial_schema.sql"},
		{Id: "002", Name: "Add users table", URL: "migrations/002_add_users_table.sql"},
		{Id: "003", Name: "Add products table", URL: "migrations/003_add_products_table.sql"},
		{Id: "004", Name: "Create indexes", URL: "migrations/004_create_indexes.sql"},
		{Id: "005", Name: "Seed data", URL: "migrations/005_seed_data.sql"},
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusAccepted)

	json.NewEncoder(w).Encode(migrations)
}

func main() {
	http.HandleFunc("/", handleGetFiles)

	fmt.Println("Server is running at http://localhost:8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
