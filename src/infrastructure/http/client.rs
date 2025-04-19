use reqwest::{Client, Error};
use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Debug)]
pub struct HttpClient {
    pub client: Client,
    pub base_url: String,
}

impl HttpClient {
    /// 새로운 HTTP 클라이언트를 생성합니다.
    ///
    /// # 인자
    ///
    /// * `base_url` - 모든 요청의 기본이 되는 베이스 URL
    ///
    /// # 반환값
    ///
    /// * `HttpClient` - 초기화된 HTTP 클라이언트 인스턴스
    ///
    /// # 예시
    /// 1) `application/json` 헤더를 기본으로 설정하는 경우
    /// ```
    /// let client = HttpClient::new("https://api.example.com");
    /// ```
    /// 2) 커스텀 헤더를 설정하는 경우
    /// ```
    /// let mut custom_headers = HeaderMap::new();
    ///
    /// custom_headers.insert("X-Custom-Header", HeaderValue::from_static("custom-value"));
    /// let another_joke: JokeResponse = client.get("/", Some(custom_headers)).await?;
    /// ```
    pub fn new(base_url: impl Into<String>) -> Self {
        HttpClient {
            client: Client::new(),
            base_url: base_url.into()
        }
    }

    /// GET 요청을 보내고 응답을 지정된 타입으로 변환합니다.
    ///
    /// # 타입 매개변수
    ///
    /// * `T` - 응답을 변환할 타입 (DeserializeOwned + Send 트레이트 구현 필요)
    ///
    /// # 인자
    ///
    /// * `endpoint` - 요청을 보낼 엔드포인트 경로 (base_url에 추가됨)
    /// * `headers` - 요청에 사용할 헤더 (None인 경우 기본 헤더 사용)
    ///
    /// # 반환값
    ///
    /// * `Result<T, reqwest::Error>` - 성공 시 응답 데이터(T 타입), 실패 시 reqwest 에러
    ///
    /// # 예시
    ///
    /// ```
    /// let user: User = client.get("/users/1", None).await?;
    /// ```
    pub async fn get<T>(
        &self,
        endpoint: &str,
        headers: Option<HeaderMap>
    ) -> Result<T, reqwest::Error>
    where
        T: DeserializeOwned + Send,
    {
        let url = format!("{}{}", self.base_url, endpoint);

        let request_headers = headers.unwrap_or_else(|| {
            let mut headers = HeaderMap::new();
            headers.insert("Accept", "application/json".parse().unwrap());
            headers
        });

        let response = self.client
            .get(&url)
            .headers(request_headers)
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(response)
    }

    /// POST 요청을 보내고 응답을 지정된 타입으로 변환합니다.
    ///
    /// # 타입 매개변수
    ///
    /// * `T` - 응답을 변환할 타입 (DeserializeOwned + Send 트레이트 구현 필요)
    /// * `B` - 요청 본문 타입 (Serialize + Send 트레이트 구현 필요)
    ///
    /// # 인자
    ///
    /// * `endpoint` - 요청을 보낼 엔드포인트 경로 (base_url 에 추가됨)
    /// * `body` - 요청 본문 데이터
    /// * `headers` - 요청에 사용할 헤더 (None인 경우 기본 헤더 사용)
    ///
    /// # 반환값
    ///
    /// * `Result<T, reqwest::Error>` - 성공 시 응답 데이터(T 타입), 실패 시 reqwest 에러
    /// * `T` - 응답 데이터 타입 (DeserializeOwned + Send 트레이트 구현한 Projection 타입)
    ///
    /// # 예시
    ///
    /// ```
    /// let new_user = CreateUser { name: "홍길동".to_string(), email: "hong@example.com".to_string() };
    /// let response: UserResponse = client.post("/users", &new_user, None).await?;
    /// ```
    pub async fn post<T, B>(
        &self,
        endpoint: &str,
        body: &B,
        headers: Option<HeaderMap>
    ) -> Result<T, Error>
    where
        T: DeserializeOwned + Send,
        B: Serialize + Send,
    {
        let url = format!("{}{}", self.base_url, endpoint);

        let request_headers = headers.unwrap_or_else(|| {
            let mut headers = HeaderMap::new();
            headers.insert("Accept", "application/json".parse().unwrap());
            headers
        });

        let response = self.client
            .post(&url)
            .headers(request_headers)
            .json(body)
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(response)
    }

    /// PUT 요청을 보내고 응답을 지정된 타입으로 변환합니다.
    ///
    /// # 타입 매개변수
    ///
    /// * `T` - 응답을 변환할 타입 (DeserializeOwned + Send 트레이트 구현 필요)
    /// * `B` - 요청 본문 타입 (Serialize + Send 트레이트 구현 필요)
    ///
    /// # 인자
    ///
    /// * `endpoint` - 요청을 보낼 엔드포인트 경로 (base_url에 추가됨)
    /// * `body` - 요청 본문 데이터
    /// * `headers` - 요청에 사용할 헤더 (None인 경우 기본 헤더 사용)
    ///
    /// # 반환값
    ///
    /// * `Result<T, reqwest::Error>` - 성공 시 응답 데이터(T 타입), 실패 시 reqwest 에러
    ///
    /// # 예시
    ///
    /// ```
    /// let update_user = UpdateUser { name: "김철수".to_string(), email: "kim@example.com".to_string() };
    /// let response: UserResponse = client.put("/users/1", &update_user, None).await?;
    /// ```
    pub async fn put<T, B>(
        &self,
        endpoint: &str,
        body: &B,
        headers: Option<HeaderMap>
    ) -> Result<T, Error>
    where
        T: DeserializeOwned + Send,
        B: Serialize + Send,
    {
        let url = format!("{}{}", self.base_url, endpoint);

        let request_headers = headers.unwrap_or_else(|| {
            let mut headers = HeaderMap::new();
            headers.insert("Accept", "application/json".parse().unwrap());
            headers
        });

        let response = self.client
            .put(&url)
            .headers(request_headers)
            .json(body)
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(response)
    }

    /// DELETE 요청을 보내고 응답을 지정된 타입으로 변환합니다.
    ///
    /// # 타입 매개변수
    ///
    /// * `T` - 응답을 변환할 타입 (DeserializeOwned + Send 트레이트 구현 필요)
    ///
    /// # 인자
    ///
    /// * `endpoint` - 요청을 보낼 엔드포인트 경로 (base_url에 추가됨)
    /// * `headers` - 요청에 사용할 헤더 (None인 경우 기본 헤더 사용)
    ///
    /// # 반환값
    ///
    /// * `Result<T, reqwest::Error>` - 성공 시 응답 데이터(T 타입), 실패 시 reqwest 에러
    ///
    /// # 예시
    ///
    /// ```
    /// // 사용자 삭제 후 성공 메시지 응답 받기
    /// #[derive(Deserialize)]
    /// struct DeleteResponse {
    ///     success: bool,
    ///     message: String,
    /// }
    ///
    /// let response: DeleteResponse = client.delete("/users/1", None).await?;
    /// if response.success {
    ///     println!("삭제 성공: {}", response.message);
    /// }
    /// ```
    pub async fn delete<T>(
        &self,
        endpoint: &str,
        headers: Option<HeaderMap>
    ) -> Result<T, Error>
    where
        T: DeserializeOwned + Send,
    {
        let url = format!("{}{}", self.base_url, endpoint);

        let request_headers = headers.unwrap_or_else(|| {
            let mut headers = HeaderMap::new();
            headers.insert("Accept", "application/json".parse().unwrap());
            headers
        });

        let response = self.client
            .delete(&url)
            .headers(request_headers)
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(response)
    }

    /// PATCH 요청을 보내고 응답을 지정된 타입으로 변환합니다.
    ///
    /// # 타입 매개변수
    ///
    /// * `T` - 응답을 변환할 타입 (DeserializeOwned + Send 트레이트 구현 필요)
    /// * `B` - 요청 본문 타입 (Serialize + Send 트레이트 구현 필요)
    ///
    /// # 인자
    ///
    /// * `endpoint` - 요청을 보낼 엔드포인트 경로 (base_url에 추가됨)
    /// * `body` - 요청 본문 데이터 (일부 필드만 업데이트할 때 사용)
    /// * `headers` - 요청에 사용할 헤더 (None인 경우 기본 헤더 사용)
    ///
    /// # 반환값
    ///
    /// * `Result<T, reqwest::Error>` - 성공 시 응답 데이터(T 타입), 실패 시 reqwest 에러
    ///
    /// # 예시
    ///
    /// ```
    /// // 사용자 정보 일부만 업데이트
    /// #[derive(Serialize)]
    /// struct PatchUser {
    ///     email: String,
    /// }
    ///
    /// let patch_data = PatchUser {
    ///     email: "new.email@example.com".to_string(),
    /// };
    ///
    /// let response: UserResponse = client.patch("/users/1", &patch_data, None).await?;
    /// println!("업데이트된 이메일: {}", response.email);
    /// ```
    pub async fn patch<T, B>(
        &self,
        endpoint: &str,
        body: &B,
        headers: Option<HeaderMap>
    ) -> Result<T, Error>
    where
        T: DeserializeOwned + Send,
        B: Serialize + Send,
    {
        let url = format!("{}{}", self.base_url, endpoint);

        let request_headers = headers.unwrap_or_else(|| {
            let mut headers = HeaderMap::new();
            headers.insert("Accept", "application/json".parse().unwrap());
            headers
        });

        let response = self.client
            .patch(&url)
            .headers(request_headers)
            .json(body)
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Post {
        #[serde(rename = "userId")]
        user_id: i32,
        id: i32,
        title: String,
        body: String,
    }
    
    #[tokio::test]
    async fn test_get() {
        let client = HttpClient::new("https://jsonplaceholder.typicode.com");
        
        let post: Post = client.get("/posts/1", None).await.unwrap();

        assert_eq!(post.id, 1);
        assert!(post.title.len() > 0);
        assert!(post.body.len() > 0);
    }

    #[tokio::test]
    async fn test_post() {
        let new_post = Post {
            user_id: 1,
            id: 0,
            title: "테스트 제목".to_string(),
            body: "테스트 내용".to_string(),
        };

        let client = HttpClient::new("https://jsonplaceholder.typicode.com");
        
        let response: Post = client.post("/posts", &new_post, None).await.unwrap();
        
        assert_ne!(response.id, 0);
        assert_eq!(response.title, new_post.title);
        assert_eq!(response.body, new_post.body);
    }

    #[tokio::test]
    async fn test_put() {
        let client = HttpClient::new("https://jsonplaceholder.typicode.com");
        
        let update_post = Post {
            user_id: 1,
            id: 1,
            title: "수정된 제목".to_string(),
            body: "수정된 내용".to_string(),
        };
        
        let response: Post = client.put("/posts/1", &update_post, None).await.unwrap();
        
        assert_eq!(response.id, 1);
        assert_eq!(response.title, update_post.title);
        assert_eq!(response.body, update_post.body);
    }

    #[tokio::test]
    async fn test_delete() {
        let client = HttpClient::new("https://jsonplaceholder.typicode.com");
        
        #[derive(Deserialize)]
        struct EmptyResponse {}

        let _: EmptyResponse = client.delete("/posts/1", None).await.unwrap();
    }

    #[tokio::test]
    async fn test_patch() {
        let client = HttpClient::new("https://jsonplaceholder.typicode.com");
        
        #[derive(Serialize)]
        struct PatchData {
            title: String,
        }

        let patch_data = PatchData {
            title: "부분 수정된 제목".to_string(),
        };
        
        let response: Post = client.patch("/posts/1", &patch_data, None).await.unwrap();
        
        assert_eq!(response.id, 1);
        assert_eq!(response.title, patch_data.title);
    }
}
