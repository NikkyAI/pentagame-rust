import { getJSONP, create_alert } from './utils.js';

document.addEventListener('DOMContentLoaded', async function () {
  let alerts = await getJSONP({ url: '/api/users/alerts', method: 'GET' });
  console.log(alerts);
  for (let i = alerts.length - 1; i > -1; i--) {
    console.log(alerts[i]);
    create_alert(alerts[i][0], alerts[i][1]);
  }
});
