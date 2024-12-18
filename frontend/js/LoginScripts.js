document.addEventListener("DOMContentLoaded", function () {
    const loginForm = document.getElementById("loginform");

    console.log(loginForm);


    // Attach an event listener to the form submission
    loginForm.addEventListener("submit", function (event) {
        event.preventDefault(); // Prevent default form submission
        handleLogin();
    });
});

function handleLogin() {
    const username = document.getElementById("username").value.trim();
    const password = document.getElementById("password").value.trim();


    if (!username || !password) {
        alert("Please enter both username and password.");
        return;
    }

    let JSONString = JSON.stringify(({
        username: username,
        password: password,
    }))

    fetch('/api/auth/login', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSONString,
    }).then((res) => res.text())  // Extract the response as plain text
        .then((data) => {
            console.log(data);  // This should log "0", "1", or "2"
            handleLoginResponse(parseInt(data));  // You can use the data in your function
        })
        .catch(function (error) {
            console.error("Error during login request:", error);
            alert("An error occurred. Please try again.");
        });



    // Send a GET request to the login API
    // fetch(`/api/login?username=${encodeURIComponent(username)}&password=${encodeURIComponent(password)}`)
    //     .then(function (response) {
    //         if (!response.ok) {
    //             throw new Error("Network response was not ok");
    //         }
    //         return response.json();
    //     })
    //     .then(function (data) {
    //         handleLoginResponse(data); // Handle the API response
    //     })
    //     .catch(function (error) {
    //         console.error("Error during login request:", error);
    //         alert("An error occurred. Please try again.");
    //     });
}

function handleLoginResponse(code) {
    switch (code) {
        case 0:
            // Login successful
            alert("Login successful!");
            window.location.href = "Home.html"; // Redirect to the home page
            break;
        case 1:
            // Username not found
            alert("No account found with this username.");
            break;
        case 2:
            // Incorrect password
            alert("Incorrect password. Please try again.");
            break;
        default:
            // Unexpected response
            alert("Unexpected response from the server.");
            console.error("Unexpected response code:", code);
            break;
    }
}
