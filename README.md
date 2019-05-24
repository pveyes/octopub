# octopub

> GitHub public data as a service

## Usage

This service provides easy way to retrieve 2 public data from GitHub without creating access token:

### Get public profile

Send GET request to `https://octopub.now.sh/profile/:username`

Example:

```sh
curl -H "Accept: application/json" https://octopub.now.sh/profile/pveyes
```

Response type:

```ts
type PublicProfileResponse = {
  username: string
  name?: string
  bio?: string
  site?: string
}
```

### Get public repository data

Send GET request to `https://octopub.now.sh/repo/:owner/:repo`

Example:

```sh
curl -H "Accept: application/json" https://octopub.now.sh/repo/pveyes/octopub
```

Response type:

```ts
type PublicRepoResponse = {
  description: string
  link?: string
  watchCount: number
  starCount: number
  forkCount: number
}
```

## Limitation

- Data retrieved using this service won't be as complete as GitHub API
- Providing same result with GitHub API is non goal

## License

MIT
