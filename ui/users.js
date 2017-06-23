$(document).ready(
    function() {
        $.get("http://localhost:3000/users", function(res) {
            var users = JSON.parse(res);
            for (index in users) {
                $("#usersTable").append(buildUserRow(users[index]));
            }
            $("#usersTable").append(buildAddUserRow());

        });
    }
);

function addUser() {
    var newUser = "{\"email\" : \"" + $("#newEmail").val() + "\", ";
    newUser += "\"first_name\" : \"" + $("#newFirstName").val() + "\", ";
    newUser += "\"last_name\" : \"" + $("#newLastName").val() + "\"}";

    $.post("/users", newUser, function(res) {
        if (res == "Ok") {
            window.location.href = "http://localhost:3000";
        } else {
            $("#newUser").append("Error");
        }
    });

}

function deleteUser(email) {
    $.ajax({
        url: 'http://localhost:3000/users',
        type: 'DELETE',
        data: email,
        success: function(result) {
            window.location.href = "http://localhost:3000";
        },
        error: function (err) {
            $("#newUser").append("Error");
        }
    });

}

function buildUserRow(user) {
    var html = "<tr>";
    html += "<td>" + user.email + "</td>";
    html += "<td>" + user.first_name + "</td>";
    html += "<td>" + user.last_name + "</td>";
    html += "<td><button class='w3-button w3-circle w3-indigo' onclick=\"deleteUser('" + user.email + "')\">x</button></td>";
    html += "</tr>";

    return html;
}

function buildAddUserRow() {
    var html = "<tr>" +
        "<td><input id='newEmail' class='w3-input' placeholder='New email'></td>" +
        "<td><input id='newFirstName' class='w3-input' placeholder='First name'></td>" +
        "<td><input id='newLastName' class='w3-input' placeholder='Last name'></td>" +
        "<td><input type='button' class='w3-button w3-circle w3-indigo' onclick=\"addUser()\" value='+'></td>" +
        "</tr>";
    return html;
}
