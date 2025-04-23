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
	File string `json:"file"`
}

func handleGetFiles(w http.ResponseWriter, r *http.Request) {
	migrations := []MigrationFileInfo{
		{Id: "001", Name: "Initial schema", File: "migrations/001_initial_schema.sql"},
		{Id: "002", Name: "Add users table", File: "migrations/002_add_users_table.sql"},
		{Id: "003", Name: "Add products table", File: "migrations/003_add_products_table.sql"},
		{Id: "004", Name: "Create indexes", File: "migrations/004_create_indexes.sql"},
		{Id: "005", Name: "Seed data", File: "migrations/005_seed_data.sql"},
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
