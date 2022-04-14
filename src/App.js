import 'regenerator-runtime/runtime'
import React from 'react'
import { utils } from 'near-api-js'
import { login, logout } from './utils'
import './global.css'

import getConfig from './config'
import { async } from 'regenerator-runtime/runtime'
const { networkId } = getConfig(process.env.NODE_ENV || 'development')

export default function App() {
  const [message, setMessage] = React.useState();
  const [topMessages, setTopMessages] = React.useState([]);
  const [buttonDisabled, setButtonDisabled] = React.useState(true);

  const getTopMessages = async () => {
    try {
      let messages = await window.contract.get_top_message();
      console.log("Top message: ", messages);
      let sorted = messages.sort((a, b) => b.created_at_block - a.created_at_block)
      setTopMessages(sorted);
    } catch (e) {
      console.log("Error: ", e);
    }
  }

  React.useEffect(() => {
    getTopMessages();
  }, []);
  

  return (
    // use React Fragment, <>, to avoid wrapping elements in unnecessary divs
    <>
      {
        window.walletConnection.isSignedIn() ? 
        <button className="link" style={{ float: 'right' }} onClick={logout}>
          Sign out
        </button>: 
        <button className="link" style={{ float: 'right' }} onClick={login}>
        Sign in
      </button>
      }
      <main>
        <h1>
          {' '/* React trims whitespace around tags; insert literal space character when needed */}
          {window.walletConnection.isSignedIn() ? `${window.accountId} !`: ""}
        </h1>
        <form onSubmit={async event => {
          event.preventDefault()
          const { fieldset, message } = event.target.elements
          const newMessage = message.value

          if (newMessage) {
            fieldset.disabled = true
            try {
              await window.contract.add_new_comment({
                message: newMessage
              }, 
              "300000000000000", // attached GAS (optional)
              "1000000000000000000000000" // 1 NEAR
              );
            } catch (e) {
              alert(
                'Something went wrong! ' +
                'Maybe you need to sign out and back in? ' +
                'Check your browser console for more info.'
              )
              throw e
            } finally {
              // re-enable the form, whether the call succeeded or failed
              fieldset.disabled = false
            }
          }
        }}>
          <fieldset id="fieldset">
            <label
              htmlFor="message"
              style={{
                display: 'block',
                color: 'var(--gray)',
                marginBottom: '0.5em'
              }}
            >
              Add new message and donate 1 NEAR:
            </label>
            <div style={{ display: 'flex' }}>
              <input
                autoComplete="off"
                id="message"
                onChange={e => setButtonDisabled(e.target.value === message)}
                style={{ flex: 1 }}
              />
              <button
                disabled={buttonDisabled}
                style={{ borderRadius: '0 5px 5px 0' }}
              >
                Save with 1 NEAR
              </button>
            </div>
          </fieldset>
        </form>
        <p>
          Top 10 new messages:
        </p>
        <ol>
          {
            topMessages.map((message, index) => {

              return (
                <li key={index}>
                  <p>
                    <strong>{message.author} </strong>
                  </p>
                  <p>
                    <code>{message.message}</code>
                  </p>
                </li>
              )

            })
          }
        </ol>
        <hr />
        <p>
          To keep learning, check out <a target="_blank" rel="noreferrer" href="https://docs.near.org">the NEAR docs</a> or look through some <a target="_blank" rel="noreferrer" href="https://examples.near.org">example apps</a>.
        </p>
      </main>
    </>
  )
}
