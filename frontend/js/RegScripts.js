document.addEventListener("DOMContentLoaded", function () {
    console.log("hello world");

    const regForm = document.getElementById("RegistrationForm");

    console.log(regForm);


    regForm.addEventListener("submit", function (event) {
        event.preventDefault();
        console.log("Inside button");
        registerAcc(regForm);

    });
});



// handle "Add User to Database" submit button click
function registerAcc(form) {

    const user = {
        username: form.querySelector('input[name="username"]').value,
        password: form.querySelector('input[name="password"]').value,
        phone: form.querySelector('input[name="mobile"]').value,
        utype: form.querySelector('select[name="UserType"]').value,
        address: {
            address_line_1: form.querySelector('input[name="address_line_1"]').value,
            address_line_2: form.querySelector('input[name="address_line_2"]').value,
            city_county: form.querySelector('input[name="city_county"]').value,
            eircode: form.querySelector('input[name="eircode"]').value,
        },
        options: "None",
    };

    console.log(user);

    let jsonString = JSON.stringify(user);

    console.log(jsonString);

    // Send a POST request to reset password
    fetch('/api/auth/signup', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: jsonString,
    }).then((res) => {
        console.log(res);
        alert("Created user successfully");
        window.location.href = "Login.HTML";
    })


}

function validateNumberInput(event) {
    // Get the value of the input
    let input = event.target;

    // Replace any non-numeric characters with an empty string
    input.value = input.value.replace(/[^0-9]/g, '');
}
