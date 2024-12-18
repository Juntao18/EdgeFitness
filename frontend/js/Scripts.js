//LogOut Function

function handleLogout() {
  const confirmLogout = confirm("Are you sure you want to log out?");
  if (confirmLogout) {
    // Clear any cookies or session storage
    document.cookie = "sessionToken=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
    localStorage.clear();

    // Redirect to login page
    window.location.href = "Login.HTML";
  }
}

function toggleSidebar() {
  var sidebar = document.getElementById("sidebar");
  sidebar.classList.toggle("expanded"); // Adds or removes 'expanded' class
}

