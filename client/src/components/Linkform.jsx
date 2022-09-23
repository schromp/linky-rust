import "../App.css";
import Alert from "./Alert";
import Newlink from "./Newlink";
import Axios from "axios";
import { useState } from "react";
import { isValidLink, isValidShortlink } from "../utilities/utils";

//TODO change to .env
const baseUrl = "http://127.0.0.1:8080/createLink";

function Linkform() {
  const [shortlink, setShortlink] = useState("");
  const [longlink, setLonglink] = useState("");

  const [slError, setSlError] = useState(false);
  const [llError, setLlError] = useState(false);

  const [gotLink, setGotLink] = useState(false);

  const handleShortlink = (event) => {
    if (isValidShortlink(event.target.value)) {
      setSlError(false)
      setShortlink(event.target.value)
    } else {
      setSlError(true)
      setShortlink(null)
    }
    setShortlink(event.target.value)
  }

  const handleLonglink = (event) => {

    if (isValidLink(event.target.value)) {
      setLlError(false)
      setLonglink(event.target.value)
    } else {
      setLlError(true)
      setLonglink(null)
    }
  }

  //TODO handle the response and implement visual feedback
  const createLink = () => {
    if (!llError && !slError) {
      Axios.post(baseUrl, {
        shortlink: shortlink,
        longlink: longlink,
      })
        .then(function (response) { console.log(response) })
        .catch(function (error) { console.log(error) })
    }
  }

  if (!gotLink) {
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

          <div>
            {llError ? <Alert message="Please input a valid link" /> : <p />}
            {slError ? <Alert message="You can only use charcters a-z and 0-1" /> : <p />}
          </div>

          <div className="form-control mt-6">
            <button className="btn btn-primary" onClick={createLink}>
              Generate
            </button>
          </div>
          <Newlink created="hello" />
        </div>
      </div>
    );
  } else {
    return <Newlink />
  }

}

export default Linkform;
