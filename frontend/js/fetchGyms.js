// Declare a global variable to store all gyms
var allGyms = [];

// Fetch gyms and display on page load
document.addEventListener("DOMContentLoaded", function () {
    fetchGyms();

    
    document.getElementById("searchInput").addEventListener("keydown", function (event) {
        if (event.key === "Enter") {
            applyFilters(); // Trigger the filter function
        }
    });
});

// Fetch gym data from backend or use sample data for testing
function fetchGyms() {
    console.log("Fetching");

    fetch('/api/gym/all')
        .then(function (response) {
            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            return response.json();
        }).then(function (data) {
            console.log(data);

            allGyms = data;
            displayGyms(allGyms);
        });
}

// Display gym data
function displayGyms(data) {
    var gymList = document.getElementById("gymList");
    gymList.innerHTML = ""; // Clear existing content

    data.forEach(function (gym) {
        var gymItem = document.createElement("div");
        gymItem.classList.add("gym-item");

        gymItem.innerHTML =
            '<img src="images/placeholder.jpg" alt="Gym Photo">' +
            "<h2>" + gym.name + "</h2>" +
            "<p><strong>Location:</strong> " + gym.location + "</p>" +
            "<p><strong>Hours:</strong> " + gym.opening_hours + " - " + gym.closing_hours + "</p>" +
            "<p><strong>Phone:</strong> " + gym.phone_number + "</p>" +
            '<button onclick="showReviews(\'' + gym.owner.$oid + '\')">More Info</button>';

        gymList.appendChild(gymItem);
    });
}

// Display reviews for a selected gym
function showReviews(gymId) {
    console.log(gymId);

    var selectedGym = allGyms.find(function (gym) {
        return gym.owner.$oid === gymId; // Match gym by ID
    });

    if (!selectedGym) {
        console.error("Gym not found with ID:", gymId);
        return;
    }

    var reviewsContent = document.getElementById("reviewsContent");
    reviewsContent.innerHTML = ""; // Clear existing content

    // Display reviews
    if (selectedGym.reviews && selectedGym.reviews.length > 0) {
        selectedGym.reviews.forEach(function (review) {
            var reviewItem = document.createElement("div");
            reviewItem.classList.add("review-item");

            reviewItem.innerHTML =
                "<p><strong>Rating:</strong> " + review.rating + "/5</p>" +
                "<p><strong>Comment:</strong> " + review.review + "</p>" +
                "<hr>";
            reviewsContent.appendChild(reviewItem);
        });
    } else {
        reviewsContent.innerHTML = "<p>No reviews available for this gym.</p>";
    }

    // Show the review section
    document.getElementById("reviewSection").style.display = "block";
}

// Close the review section
function closeReviewSection() {
    document.getElementById("reviewSection").style.display = "none";
}

// Apply filters to the gym list
function applyFilters() {
    var searchInput = document.getElementById("searchInput").value.toLowerCase();
    var filterOption = document.getElementById("filterOptions").value;

    // Filter gyms by search and selected condition
    var filteredGyms = allGyms.filter(function (gym) {
        var matchesSearch =
            gym.name.toLowerCase().includes(searchInput) ||
            gym.location.toLowerCase().includes(searchInput);

        var matchesFilter = true;
        if (filterOption === "morning") {
            matchesFilter = parseInt(gym.opening_hours.split(":")[0]) < 12;
        } else if (filterOption === "evening") {
            matchesFilter = parseInt(gym.closing_hours.split(":")[0]) > 18;
        }

        return matchesSearch && matchesFilter;
    });

    displayGyms(filteredGyms); // Update displayed gyms
}
