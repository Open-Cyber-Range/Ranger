import Keycloak from 'keycloak-js';
import {isDevelopment} from 'src/utils';

const keycloak = new Keycloak(isDevelopment() ? {
  url: 'http://localhost:8080',
  realm: 'OCR',
  clientId: 'ranger-ui',
} : undefined);

export default keycloak;
