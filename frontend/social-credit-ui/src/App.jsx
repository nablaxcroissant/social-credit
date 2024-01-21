import { useState } from 'react'
import useSWR from 'swr'
import './App.css'

const fetcher = (...args) => fetch(...args).then((res)=> res.json());

function App() {
  const {
    data: data,
    error,
    isValidating,
  } = useSWR('http://localhost:3000/api/get_scores', fetcher);


  if (error) return <div className='failed'>failed to load</div>;
  if (isValidating) return <div className="Loading">Loading...</div>;

  return (
    <>
      <h2>
        Social Credit Scores
      </h2>
      <table>
        <tbody>
        {data &&
        data.people.map((person, index) => (
          <tr key={person.last_name}>
            <td>{person.first_name +' '+ person.last_name}</td>
            <td>{person.score}</td>
          </tr>
        ))}
        </tbody>
      </table>
      <h2>Report comrade for good or bad behavior</h2>
      <table>
        <tr>
          <td>
            <label for="first_name">First Name</label></td>
          <td>
          <input type="text" id="first_name" name="first_name"/>
          </td> 
        </tr>

        <tr>
          <td>
          <label for="last_name">Last Name</label>
          </td>
          <td>
          <input type="text" id="last_name" name="last_name"/>
          </td>
        </tr>

        <tr>
          <td>
          <label for="update">Points taken/given</label>
          </td>
          <td>
          <input type="number" id="update" name="update"/>
          </td>
        </tr>

        <tr>
          <td>
          <label for="reason">Reason</label></td>
          <td>
          <input type="text" id="reason" name="reason"/>
          </td>
        </tr>
      </table>
      <button>Submit!</button>
      </>
  )
}

export default App
