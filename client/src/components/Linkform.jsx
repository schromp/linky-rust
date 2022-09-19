import "../App.css";
import Alert from "./Alert";
import Axios from "axios";
import { useState } from "react";

//TODO change to .env
const baseUrl = "http://127.0.0.1:8080/createLink";

function Linkform() {
  const [shortlink, setShortlink] = useState("");
  const [longlink, setLonglink] = useState("");
  
  const handleShortlink = (event) => {
    setShortlink(event.target.value)
  }

  const handleLonglink = (event) => {
    setLonglink(event.target.value)
  }

  //TODO handle the response and implement visual feedback
  const createLink = () => {
    console.log(shortlink + " " + longlink)
    Axios.post(baseUrl, {
      shortlink: shortlink,
      longlink: longlink,
    })
    .then(function (response) {console.log(response)})
    .catch(function (error) {console.log(error)})
  }

  return (
    <div className="card flex-shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
      <div className="card-body">
        <div className="form-control">
          <label className="label">
            <span className="label-text">Your very long link</span>
          </label>
          <input
            type="text"
            placeholder="https://verylonglink.com/very/long/link"
            className="input input-bordered"
            onChange={handleLonglink}
          />

          <label className="label">
            <span className="label-text">
              Personalized Link: Leave empty to autogenerate
            </span>
          </label>
          <label className="input-group">
            <span>schro.it/</span>
            <input
              type="text"
              placeholder="short link"
              className="input input-bordered w-full"
              onChange={handleShortlink}
            />
          </label>
        </div>

        <Alert />

        <div className="form-control mt-6">
          <button className="btn btn-primary" onClick={createLink}>
            Generate
          </button>
        </div>
      </div>
    </div>
  );
}

export default Linkform;
