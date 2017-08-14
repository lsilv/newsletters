$(document).ready(
    function() {
        $.get("/templates", function(res) {
            var templates = JSON.parse(res).templates;
            for (index in templates) {
                $("#templates").append(buildTemplateButton(templates[index].name + " template", templates[index].id));
            }
            $("#templates").append(buildAddTemplateButton());
        });

    }
);

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
        url: '/templates/' + templateId,
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
        $("#name").val(template.name);
        $("#content").val(template.versions[0].plain_content);
        $("#versionid").val(template.versions[0].id);
    }
}

function editTemplate() {
    var request = "{ \"subject\": \"" + $("#subject").val() + "\", ";
    request += "\"content\": \"" + $("#content").val() + "\", ";
    request += "\"name\": \"" + $("#name").val() + "\", ";
    request += "\"version_id\": \"" + $("#versionid").val() + "\"}";
    request = request.replace('\n', '\\n');

    $.ajax({
        url: '/templates/' + $("#templateid").val(),
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
    request = request.replace('\n', '\\n');

    $.ajax({
        url: '/templates',
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
        url: '/templates/' + $("#templateid").val() + '/' + $("#versionid").val(),
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
    $.ajax({
        url: '/send/' + $("#templateid").val(),
        type: 'POST',
        success: function(data, status) {
            $("#template").append(status + data);
        },
        error: function (err, status) {
            $("#template").append("Invalid emails");
        }
    });
}