$(document).ready(
    function() {
        $.get("http://localhost:3000/users", function(res) {
            var users = JSON.parse(res);
            for (index in users) {
                $("#usersTable").append(buildUserRow(users[index]));
            }
        });


        $("#add").click(function () {
            var newUser = "{\"email\" : \"" + $("#newEmail").val() + "\", ";
            newUser += "\"first_name\" : \"" + $("#newFirstName").val() + "\", ";
            newUser += "\"last_name\" : \"" + $("#newLastName").val() + "\"}";

            $.post("/users", newUser, function(res) {
                if (res == "Ok") {
                    window.location.href = "http://localhost:3000/index";
                } else {
                    $("#newUser").append("Error");
                }
            });
        });
    }
);

function deleteUser(email) {
    $.ajax({
        url: 'http://localhost:3000/users',
        type: 'DELETE',
        data: email,
        success: function(result) {
            window.location.href = "http://localhost:3000/index";
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
    html += "<td><button class='w3-button w3-circle w3-hover-indigo' onclick=\"deleteUser('" + user.email + "')\"><b>x</b></button></td>";
    html += "</tr>";

    return html;
}
