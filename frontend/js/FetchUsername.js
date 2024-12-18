document.addEventListener("DOMContentLoaded", function () {
    // Fetch username from backend
    fetchUsername();
});

function fetchUsername() {
    // backend call to fetch username
    fetch('api/user/search/6747b9305062eb229f7315ee') //  backend API
        .then(function (response) {
            if (!response.ok) {
                throw new Error('Failed to fetch user data');
            }
            return response.json();
        })
        .then(function (data) {
            console.log(data);

            const username = data[0].username || "User"; // Use "User" as fallback
            document.getElementById("welcome-text").textContent = `Welcome! ${username}`;
        })
        .catch(function (error) {
            console.error('Error fetching username:', error);
            // Fallback to a default username if API call fails
            document.getElementById("welcome-text").textContent = "Welcome! User";
        });
}
