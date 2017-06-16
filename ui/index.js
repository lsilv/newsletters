$(document).ready(
    function() {
        $.get("http://localhost:3000/users", function(res) {
            var users = JSON.parse(res);
            for (index in users) {
                $("#usersTable").append(buildUserRow(users[index]));
            }
            $("#usersTable").append(buildAddUserRow());

        });

        $.get("http://localhost:3000/templates", function(res) {
            var templates = JSON.parse(res).templates;
            for (index in templates) {
                $("#templates").append(buildTemplateButton(templates[index].name + " template", templates[index].id));
            }
            $("#templates").append(buildAddTemplateButton());
        });

   }
);

function addUser() {
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

}

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

function buildTemplateButton(name, templateId) {
    var html = "<button onclick='showTemplate(\""+ templateId +"\")' class='w3-button w3-margin-top w3-indigo w3-block w3-left-align' id='" + templateId + "'>" + name + "</button>";
    html += "<p></p>";
    return html;
}

function buildAddTemplateButton() {
    var html = "<button onclick='showNewTemplateForm()' class='w3-button w3-margin-top w3-indigo w3-block w3-left-align'>Add a new template</button>";
    html += "<p></p>";
    return html;
}

function showNewTemplateForm() {
    document.getElementById('template').style.display = 'none';
    document.getElementById('newTemplate').style.display = 'block';
}

function showTemplate(templateId) {
    $.ajax({
        url: 'http://localhost:3000/templates/' + templateId,
        type: 'GET',
        success: function(result) {
            $("#template").append(setTemplate(JSON.parse(result)));
        },
        error: function (err) {
            $("#template").append("Error");
        }
    });
}

function setTemplate(template) {
    if ("error" in template) {
        $("#template").append("Error: " + template);
    } else {
        document.getElementById('template').style.display = 'block';
        document.getElementById('newTemplate').style.display = 'none';
        $("#templateid").val(template.id);
        $("#subject").val(template.versions[0].subject);
        $("#content").val(template.versions[0].plain_content);
        $("#versionid").val(template.versions[0].id);
    }
}

function editTemplate() {
    var request = "{ \"subject\": \"" + $("#subject").val() + "\", ";
    request += "\"content\": \"" + $("#content").val() + "\", ";
    request += "\"name\": \"" + "Name" + "\", ";
    request += "\"version_id\": \"" + $("#versionid").val() + "\"}";
    request = request.replace('\n', '\\n');

    $.ajax({
        url: 'http://localhost:3000/templates/' + $("#templateid").val(),
        type: 'PUT',
        data: request,
        success: function(result) {
            $("#template").append(setTemplate(JSON.parse(result)));
        },
        error: function (err) {
            $("#template").append("Error: ");
        }
    });
}

function addTemplate() {
    var request = "{ \"name\": \"" + $("#newName").val() + "\",";
    request += "\"content\": \"" + $("#newContent").val() + "\", ";
    request += "\"version_id\": \"no\",";
    request += "\"subject\": \"" + $("#newSubject").val() + "\"}";
    
    $.ajax({
        url: 'http://localhost:3000/templates',
        type: 'POST',
        data: request,
        success: function(result) {
            location.reload();
        },
        error: function (err) {
            $("#newTemplate").append("Error: ");
        }
    });
}

function deleteTemplate() {
    $.ajax({
        url: 'http://localhost:3000/templates/' + $("#templateid").val(),
        type: 'DELETE',
        success: function(result) {
            location.reload();
        },
        error: function (err) {
            $("#template").append("Error: ");
        }
    });
}

function sendEmails() {
    alert("Not implemented");
}