package main

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"time"

	"github.com/google/uuid"
	"github.com/jackc/pgx/v5"
)

type Ping struct {
	Id         string    `json:"id"`
	Service    string    `json:"service"`
	Amount     int       `json:"amount"`
	Created_at time.Time `json:"created_at"`
	Updated_at time.Time `json:"updated_at"`
}

func main() {
	DEFAULT_PORT := "8080"

	port := os.Getenv("PORT")
	databaseUrl := os.Getenv("DATABASE_URL")

	if port == "" {
		port = DEFAULT_PORT
	}

	if databaseUrl == "" {
		panic("Database URL wasn't specified")
	}

	conn, err := pgx.Connect(context.Background(), databaseUrl)

	if err != nil {
		panic(fmt.Errorf("error on connecting to DB: %w", err))
	}

	defer conn.Close(context.Background())

	http.HandleFunc("/ping", func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			w.WriteHeader(http.StatusMethodNotAllowed)

			return
		}

		var ping Ping

		err := conn.QueryRow(context.Background(), "SELECT * FROM pings WHERE service=$1", "go").Scan(&ping)

		if err != nil && err == pgx.ErrNoRows {
			id, _ := uuid.NewV7()
			var ping Ping

			err := conn.QueryRow(context.Background(), `
                INSERT INTO pings (id, service, amount)
                VALUES ($1, $2, $3)
                RETURNING id, service, amount, created_at, updated_at
            `, id, "go", 1).Scan(&ping.Id, &ping.Service, &ping.Amount, &ping.Created_at, &ping.Updated_at)

			if err != nil {
				http.Error(w, "error on creating record", http.StatusInternalServerError)

				return
			}

			w.Header().Set("Content-Type", "application/json")
			w.WriteHeader(http.StatusCreated)
			json.NewEncoder(w).Encode(ping)

		} else {
			var ping Ping

			err := conn.QueryRow(context.Background(), `
                UPDATE pings
                SET amount = amount + 1, updated_at = NOW()
                WHERE service=$1
                RETURNING id, service, amount, created_at, updated_at
            `, "go").Scan(&ping.Id, &ping.Service, &ping.Amount, &ping.Created_at, &ping.Updated_at)

			if err != nil {
				http.Error(w, "error on updating record", http.StatusInternalServerError)

				return
			}

			w.Header().Set("Content-Type", "application/json")
			w.WriteHeader(http.StatusOK)
			json.NewEncoder(w).Encode(ping)
		}
	})

	http.HandleFunc("/health", func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodGet {
			w.WriteHeader(http.StatusMethodNotAllowed)

			return
		}

		err := conn.Ping(context.Background())

		if err != nil {
			http.Error(w, "", http.StatusServiceUnavailable)

			return
		}

		w.WriteHeader(http.StatusOK)
	})

	fmt.Printf("🚀 App running at PORT: %s\n", port)

	if err := http.ListenAndServe("0.0.0.0:"+port, nil); err != nil {
		panic(err)
	}
}
