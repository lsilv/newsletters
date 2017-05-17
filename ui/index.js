$(document).ready(
    function() {
        $.get("http://localhost:3000/users", function(res) {
            var users = JSON.parse(res);
            for (index in users) {
                $("#usersTable").append(buildUserRow(users[index]));
            }
            $("#usersTable").append(buildAddRow());
        });


        $("#add").click(function () {
            var newEmail = $("#email").val();
            $.post("http://localhost:3000/users", "{\"email\" : newEmail}", function(res) {
                        $("#users").text(res);
                    });
        });
    }
);

function buildUserRow(user) {
    var html = "<tr>";
    html += "<td>" + user.email + "</td>";
    html += "<td>" + user.first_name + "</td>";
    html += "<td>" + user.last_name + "</td>";
    html += "<td><button class=\"w3-button w3-circle w3-hover-indigo\"><b>x</b></button></td>";
    html += "</tr>";

    return html;
}

function buildAddRow() {
    return "<tr>" +
                "<td><input class=\"w3-input\" placeholder=\"New email\"></td>" +
                "<td><input class=\"w3-input\" placeholder=\"First name\"></td>" +
                "<td><input class=\"w3-input\" placeholder=\"Last name\"></td>" +
                "<td><button class=\"w3-button w3-circle w3-hover-indigo\" id=\"Add\"><b>+</b></button></td>" +
            "</tr>";
}