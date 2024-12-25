function renderItems(items, processType, elementId, processFunction) {
  let itemsMeta = [];
  let placeholder = "<div>";
  for (let i = 0; i < items.length; i++) {
    let title = items[i].title;
    let placeholderId = `${processType}-${title.replaceAll(" ", "-")}`;
    placeholder += `
        <div id="${placeholderId}" class="itemContainer">
            <p>${title}</p>
            <button class="actionButton" id="${placeholderId}">${processType}</button>        
        </div>
    `;
    itemsMeta.push({ id: placeholderId, title: title });
  }
  placeholder += "</div>";
  document.getElementById(elementId).innerHTML = placeholder;
  for (let i = 0; i < itemsMeta.length; i++) {
    document
      .getElementById(itemsMeta[i].id)
      .addEventListener("click", processFunction);
  }
}

function apiCall(url, method) {
  let xhr = new XMLHttpRequest();
  xhr.withCredentials = true;
  xhr.addEventListener("readystatechange", function () {

    document.getElementById("completeNum").innerHTML = JSON.parse(this.responseText)["done_item_count"];
    document.getElementById("pendingNum").innerHTML = JSON.parse(this.responseText)["pending_item_count"];
    if (this.readyState === this.DONE) {
      renderItems(
        JSON.parse(this.responseText).pending_items,
        "edit",
        "pendingItems",
        editItem
      );

      renderItems(
        JSON.parse(this.responseText).done_items,
        "delete",
        "doneItems",
        deleteItem
      );
    }
  });
  xhr.open(method, url);
  xhr.setRequestHeader("content-type", "application/json");
  xhr.setRequestHeader("token", localStorage.getItem("token"));
  return xhr;
}

function editItem() {
  let title = this.id.replaceAll("-", " ").replace("edit ", "");
  let call = apiCall(`/v1/item/edit`, "POST");
  let json = {
    title: title,
    status: "DONE"
  };
  call.send(JSON.stringify(json));
}

function deleteItem() {
  let title = this.id.replaceAll("-", " ").replace("delete ", "");
  let call = apiCall(`/v1/item/delete`, "POST");
  let json = {
    title: title,
    status: "DONE"
  };
  call.send(JSON.stringify(json));
}

function getItems() {
  let call = apiCall(`/v1/item/get`, "GET");
  call.send();
}

getItems();
document.getElementById("create-button").addEventListener("click", createItem);
if (localStorage.getItem("token")) {
  document.getElementById("header-login").style.display = "none";
  document.getElementById("header-logout").style.display = "block";
} else {
  document.getElementById("header-login").style.display = "block";
  document.getElementById("header-logout").style.display = "none";
}

function createItem() {
  let title = document.getElementById("name");
  let call = apiCall(`/v1/item/create/${title.value}`, "POST");
  call.send();
  document.getElementById("name") = value = null;
}

function onLogin(e) {
  e.preventDefault();
  e.stopPropagation();
  const name = document.getElementById("login-input-name").value;
  const passwd = document.getElementById("login-input-password").value;
  let xhr = new XMLHttpRequest();
  xhr.withCredentials = true;
  xhr.addEventListener("readystatechange", function () {

    const data = xhr.getResponseHeader("Token");
    if (data) {
      localStorage.setItem("token", data);
      document.getElementById("loginModal").style.display = "none";
      window.location.reload();
    }

  });
  xhr.open("POST", "/v1/auth/login");
  xhr.setRequestHeader("content-type", "application/json");
  xhr.send(JSON.stringify({ username: name, password: passwd }));

}

function onLogout(e) {
  console.log("e==>", e);
  e.preventDefault();
  e.stopPropagation();
  localStorage.removeItem("token");
  window.location.replace(window.location.origin);
}
