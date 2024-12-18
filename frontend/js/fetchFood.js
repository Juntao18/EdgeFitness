//  Global variable to store all food company data
var allFoods;

//  Fetch data on page load
document.addEventListener("DOMContentLoaded", function () {
    fetchFoods();

    document.getElementById("searchInput").addEventListener("keydown", function (event) {
        if (event.key === "Enter") {
            applyFilters();
        }
    });
});

//  Fetch food company data from the backend
function fetchFoods() {
    fetch('/api/provider/all') //  API!!!
        .then(function (response) {
            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            return response.json();
        })
        .then(function (data) {
            allFoods = data;
            displayFoods(allFoods);
        })
        .catch(function (error) {
            console.error('Error fetching food providers:', error);
        });
}

// Display food company data
function displayFoods(data) {
    var foodList = document.getElementById("foodList");
    foodList.innerHTML = ''; //Clear existing content

    data.forEach(function (food) {
        var foodItem = document.createElement("div");
        foodItem.classList.add("gym-item");

        foodItem.innerHTML =
            '<img src="images/foodplaceholder.jpg" alt="Food Provider Logo">' +
            '<h2>' + food.name + '</h2>' +
            '<p><strong>Location:</strong> ' + food.location + '</p>' +
            '<p><strong>Hours:</strong> ' + food.opening_hours + ' - ' + food.closing_hours + '</p>' +
            '<p><strong>Phone:</strong> ' + food.phone_number + '</p>' +
            '<button onclick="showReviews(' + food.id + ')">More Info</button>';

        foodList.appendChild(foodItem);
    });
}


function showReviews(foodId) {
    var selectedFood = allFoods.find(function (food) {
        return food.id === foodId;
    });

    if (!selectedFood) {
        console.error("Food provider not found with ID:", foodId);
        return;
    }

    var reviewsContent = document.getElementById("reviewsContent");
    reviewsContent.innerHTML = "";

    if (selectedFood.reviews && selectedFood.reviews.length > 0) {
        selectedFood.reviews.forEach(function (review) {
            var reviewItem = document.createElement("div");
            reviewItem.classList.add("review-item");

            reviewItem.innerHTML =
                `<p><strong>Rating:</strong> ${review.rating}/5</p>
                 <p><strong>Comment:</strong> ${review.review}</p>
                 <hr>`;
            reviewsContent.appendChild(reviewItem);
        });
    } else {
        reviewsContent.innerHTML = "<p>No reviews available for this provider.</p>";
    }

    document.getElementById("reviewSection").style.display = "block";
}

function closeReviewSection() {
    document.getElementById("reviewSection").style.display = "none";
}
// Filtering Functionality
function applyFilters() {
    var searchInput = document.getElementById("searchInput").value.toLowerCase();
    var filterOption = document.getElementById("filterOptions").value;

    var filteredFoods = allFoods.filter(function (food) {
        var matchesSearch =
            food.name.toLowerCase().includes(searchInput) ||
            food.location.toLowerCase().includes(searchInput);

        var matchesFilter = true;
        if (filterOption === "morning") {
            matchesFilter = parseInt(food.opening_hours.split(":")[0]) < 12;
        } else if (filterOption === "evening") {
            matchesFilter = parseInt(food.closing_hours.split(":")[0]) > 18;
        }

        return matchesSearch && matchesFilter;
    });

    displayFoods(filteredFoods);
}
