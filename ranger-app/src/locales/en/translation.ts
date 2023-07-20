const language = {
  homepage: 'Welcome to Ranger!',
  appName: 'Ranger',
  exercises: {
    title: 'Exercises',
    add: 'Add exercise',
    name: 'Exercise name',
    scenarioSDL: 'Scenario SDL',
    sdlGuide: 'SDL Reference Guide',
    sdlParserInitFail: 'SDL Parser Initialization Failed',
    sdlParsingFail: 'SDL Parsing failed',
    sdlNotSaved: 'You have unsaved changes in SDL',
    addingSuccess: 'Exercise {{exerciseName}} added',
    addingFail: 'Failed to add the exercise',
    updateSuccess: 'Exercise {{exerciseName}} updated',
    updateFail: 'Failed to update the exercise: {{errorMessage}}',
    noDeployments: 'No deployments',
    noDeploymentInfo: 'No deployment info',
    deleteSuccess: 'Exercise {{exerciseName}} deleted',
    deleteFail: 'Failed to delete the exercise {{exerciseName}}',
    easeDevelopment: 'To ease the development of the exercise open: ',
    mustHaveName: 'Exercise must have a name',
    tabs: {
      dashboard: 'Dashboard',
      scores: 'Scores',
    },
  },
  deployments: {
    title: 'Deployments',
    add: 'Add new',
    create: 'Create a new deployment',
    addingSuccess: 'Deployment {{newDeploymentName}} added',
    addingFail: 'Failed to add the deployment',
    sdlMissing: 'Exercise must have an sdl-schema',
    deleteSuccess: 'Deployment {{deploymentName}} deleted',
    deleteFail: 'Failed to delete the deployment',
    noDeployments: 'No deployments',
    noAccounts: 'No accounts',
    entityConnector: {
      selectEntity: 'Select entity...',
      selectUser: 'Select user...',
    },
    form: {
      group: {
        placeholder: 'Select group...',
        title: 'Deployment group',
        required: 'Deployment group is required',
      },
      name: {
        title: 'Deployment name',
        required: 'Deployment name is required',
      },
      count: {
        title: 'Deployment count',
        required: 'Deployment count is required',
      },
    },
  },
  menu: {
    home: 'Home',
    exercises: 'Exercises',
    logout: 'Logout',
    greeting: 'Hi, {{username}}!',
    userRoles: {
      'ranger-manager': 'Manager',
      'ranger-participant': 'Participant',
    },
    noRole: 'NO ROLE',
  },
  common: {
    connect: 'Connect',
    required: '(required)',
    submit: 'Submit',
    delete: 'Delete',
    back: 'Back',
    deleting: 'Deleting',
    add: 'Add',
    virtualMachines: 'Virtual Machines',
    switches: 'Switches',
    templates: 'Templates',
    team: 'Team',
    noResults: 'No results',
    adGroup: 'AD Group',
  },
  emails: {
    link: 'Emails',
    emailLog: 'Email log:',
    status: 'Status',
    timestamp: 'Timestamp',
    toEntity: 'To Entity',
    from: 'From',
    to: 'To',
    replyTo: 'Reply to',
    subject: 'Subject',
    bcc: 'Bcc',
    cc: 'Cc',
    body: 'Body',
    send: 'Send',
    sendingSuccess: 'Email sent',
    sendingFail: 'Error trying to send the email: {{errorMessage}}',
    sendingFailWithoutMessage: 'Failed to send the email',
    invalidEmailAddress: 'Invalid email address(es): {{invalidEmailAddresses}}',
    form: {
      from: {
        title: 'From',
      },
      to: {
        title: 'To',
        required: 'To address is required',
      },
      replyTo: {
        title: 'Reply to',
      },
      cc: {
        title: 'Cc',
      },
      bcc: {
        title: 'Bcc',
      },
      subject: {
        title: 'Email subject',
        required: 'Email subject is required',
      },
      body: {
        title: 'Email body',
        required: 'Email body is required',
      },
      emailPlaceholder:
      'Enter email address, multiple addresses separated by comma',
    },
  },
  tloTable: {
    headers: {
      tlo: 'Training Learning Objective',
      evaluation: 'Evaluation',
      metric: 'Metric - VM Name: Current Score',
    },
    points: 'points',
    noEvaluations: 'No Evaluations to display',
    noTlos: 'No Training Learning Objectives to display',
    noMetricData: 'No metric scores to display',
  },
  chart: {
    scoring: {
      title: 'Score',
      xAxisTitle: 'Time',
      yAxisTitle: 'Points',
      noScoreData: 'No score data to display graph',
    },
  },
  participant: {
    exercise: {
      tabs: {
        dash: 'Dashboard',
        score: 'Score',
        events: 'Events',
        accounts: 'Accounts',
      },
      events: {
        noEvents: 'Participant is not connected to any Events in this deployment',
        noTriggeredEvents: 'No Events have been triggered yet',
        noDescription: 'Event has no description',
        triggeredAt: 'Event triggered at',
      },
    },
  },
};

export default language;
