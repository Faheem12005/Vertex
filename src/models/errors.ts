export type ErrorKind =
    | { kind: 'authError'; message: string }
    | { kind: 'invalidJsonFormat'; message: string }
    | { kind: 'networkError'; message: string }
    | { kind: 'storeError'; message: string }
    | { kind: 'invalidFormat'; message: string }
    | { kind: 'credentialsError'; message: string }
    | { kind: 'invalidRequestError'; message: string };

