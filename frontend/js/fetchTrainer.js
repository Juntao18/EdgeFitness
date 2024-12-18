var allTrainers;

// Fetch trainers and display them
document.addEventListener("DOMContentLoaded", function () {
    fetchTrainers();

    document.getElementById("searchInput").addEventListener("keydown", function (event) {
        if (event.key === "Enter") {
            applyFilters();
        }
    });
});

function fetchTrainers() {
    fetch('/api/user/trainer/all') // API for trainers
        .then(function (response) {
            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            return response.json();
        })
        .then(function (data) {
            allTrainers = data;
            displayTrainers(allTrainers);
            populateSpecialtyFilters(allTrainers);
        })
        .catch(function (error) {
            console.error('Error fetching trainers:', error);
        });
}

function displayTrainers(data) {
    console.log(data);

    var trainerList = document.getElementById("trainerList");
    trainerList.innerHTML = "";

    data.forEach(function (trainer) {
        var trainerItem = document.createElement("div");
        trainerItem.classList.add("gym-item");

        trainerItem.innerHTML =
            `<img src="images/trainerplaceholder.png" alt="Trainer Photo">
             <h2>${trainer.options.StringVector[0][0]}</h2>
             <p><strong>Specialty:</strong> ${trainer.options.StringVector[0][1]}</p>
             <p><strong>Phone:</strong> ${trainer.phone}</p>
             <button onclick="showReviews('${trainer._id.$oid}')">More Info</button>`;

        trainerList.appendChild(trainerItem);
    });
}

function populateSpecialtyFilters(trainers) {
    const filterOptions = document.getElementById("filterOptions");

    const specialties = new Set(
        trainers.map(trainer => trainer.options?.StringVector?.[0]?.[1]).filter(Boolean)
    );

    filterOptions.innerHTML = `<option value="all">All</option>`;
    specialties.forEach(specialty => {
        const option = document.createElement("option");
        option.value = specialty.toLowerCase();
        option.textContent = specialty;
        filterOptions.appendChild(option);
    });
}

function showReviews(trainerId) {
    console.log(trainerId);

    var selectedTrainer = allTrainers.find(function (trainer) {
        return trainer._id.$oid === trainerId;
    });

    if (!selectedTrainer) {
        console.error("Trainer not found with ID:", trainerId);
        return;
    }

    console.log();


    var reviewsContent = document.getElementById("reviewsContent");
    reviewsContent.innerHTML = "";

    if (selectedTrainer.options.StringVector[1] && selectedTrainer.options.StringVector[1].length > 0) {
        selectedTrainer.options.StringVector[1].forEach(function (review) {
            var reviewItem = document.createElement("div");
            reviewItem.classList.add("review-item");

            reviewItem.innerHTML =
                `<p><strong>Rating:</strong> ${review.rating}/5</p>
                 <p><strong>Comment:</strong> ${review.review}</p>
                 <hr>`;
            reviewsContent.appendChild(reviewItem);
        });
    } else {
        reviewsContent.innerHTML = "<p>No reviews available for this trainer.</p>";
    }

    document.getElementById("reviewSection").style.display = "block";
}

function closeReviewSection() {
    document.getElementById("reviewSection").style.display = "none";
}


function applyFilters() {
    const searchInput = document.getElementById("searchInput").value.toLowerCase();
    const selectedFilter = document.getElementById("filterOptions").value;

    const filteredList = allTrainers.filter(trainer => {
        const trainerName = trainer.options?.StringVector?.[0]?.[0]?.toLowerCase() || "";
        const trainerSpecialty = trainer.options?.StringVector?.[0]?.[1]?.toLowerCase() || "";

        const matchesSearch = trainerName.includes(searchInput);
        const matchesFilter = selectedFilter === "all" || trainerSpecialty === selectedFilter;

        return matchesSearch && matchesFilter;
    });
    displayTrainers(filteredList);
}

