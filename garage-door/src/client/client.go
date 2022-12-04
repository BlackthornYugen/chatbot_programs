package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"time"
)

type GarageDoor struct {
	ID     uint64 `json:"id"`
	Status string `json:"status"`
}

type OpenDoorRequest struct {
	Pin   string        `json:"pin"`
	Delay time.Duration `json:"delay"`
}

func main() {
	// Create a new client.
	client := &http.Client{}

	// Get the status of door 1.
	req, err := http.NewRequest("GET", "http://localhost:8080/api/garage/status/1", nil)
	if err != nil {
		panic(err)
	}

	resp, err := client.Do(req)
	if err != nil {
		panic(err)
	}

	var door GarageDoor
	if err := json.NewDecoder(resp.Body).Decode(&door); err != nil {
		panic(err)
	}

	fmt.Println("Door status:", door.Status)

	// Open door 1 and keep it open for 5 seconds.
	req, err = http.NewRequest("POST", "http://localhost:8080/api/garage/open/1", nil)
	if err != nil {
		panic(err)
	}

	req.Header.Set("Content-Type", "application/json")

	openReq := OpenDoorRequest{
		Pin:   "1234",
		Delay: 5 * time.Second,
	}

	if err := json.NewEncoder(req.Body).Encode(&openReq); err != nil {
		panic(err)
	}

	resp, err = client.Do(req)
	if err != nil {
		panic(err)
	}

	// Wait for the door to automatically close.
	time.Sleep(6 * time.Second)

	// Get the updated status of door 1.
	req, err = http.NewRequest("GET", "http://localhost:8080/api/garage/status/1", nil)
	if err != nil {
		panic(err)
	}

	resp, err = client.Do(req)
	if err != nil {
		panic(err)
	}

	if err := json.NewDecoder(resp.Body).Decode(&door); err != nil {
		panic(err)
	}

	fmt.Println("Door status:", door.Status)
}
