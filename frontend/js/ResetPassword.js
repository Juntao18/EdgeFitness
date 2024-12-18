document.addEventListener("DOMContentLoaded", function () {
    const resetPasswordForm = document.getElementById("resetPasswordForm");

    resetPasswordForm.addEventListener("submit", function (event) {
        event.preventDefault(); // Prevent default form submission
        handleResetPassword();
    });
});

function handleResetPassword() {
    const username = document.getElementById("username").value.trim();
    const newPassword = document.getElementById("newPassword").value.trim();
    const confirmPassword = document.getElementById("confirmPassword").value.trim();

    // Validate password match
    if (newPassword !== confirmPassword) {
        document.getElementById("responseMessage").textContent = "Passwords do not match!";
        return;
    }
    let test = JSON.stringify({
        username: username,
        newPassword: newPassword
    });

    console.log(test);

    // Send a POST request to reset password
    fetch('/api/user/reset-password', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: test,
    })
        .then(function (response) {
            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            console.log(response);
            return response.json();

        })
        .then(function (data) {

            console.log(data);

            if (data.success) {
                alert("Password reset successfully! Redirecting to login page...");
                window.location.href = "Login.HTML";
            } else {
                document.getElementById("responseMessage").textContent = data.message || "An error occurred.";
            }
        })
        .catch(function (error) {
            console.error("Error:", error);
            document.getElementById("responseMessage").textContent = "An error occurred. Please try again.";
        });
}
