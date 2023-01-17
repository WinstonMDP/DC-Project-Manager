#include <iostream>
#include <vector>
using namespace std;

string first_lexem[] = {"sign", "collection", "create", "add", "edit", "remove", "document", "documents"};

string second_lexem[] = {"up", "in", "out", "collection", "document"};

string third_lexem[] = {"description", "action", "field"};

void SignUp(string name, string email, string password){

}

void SignIn(string login, string password){

}

void CollectionIn(uint collectionId){

}

void CollectionOut(){

}

void CreateCollection(){

}

void EditCollectionDescription(string description){

}

void EditCollectionAction(string file_name){

}

void RemoveCollectionAction(string file_name){

}

void WorkWithCollection(string collection){

}

void CreateDocument(){

}

void ShowDocuments() {

}

void ShowDocument(string name){

}

void ShowRandomDocument(){

}

void AddDocumentField(string field){

}
void RemoveDocumentField(string field){

}

void EditDocumentField(string field, string value){

}


void SignOut(){

}



int main(int argc, char *argv[]) {
    vector<string> words;
    vector<char> current_sequence;
    int words_count = 0;
    int size = sizeof(*argv) / sizeof(*argv);

    for(int i = 0; i < size;) {
        char curr = *argv[i];
        if (curr == ' ') {
            string str(current_sequence.begin(), current_sequence.end());
            words.push_back(str);
            current_sequence = {};
        } else {
            current_sequence.push_back(curr);
        }
        i++;
    };

    if (current_sequence.size() != 0){
        string str(current_sequence.begin(), current_sequence.end());
        words.push_back(str);
    }
    if(words.size() == 1){
        if(words[0] == "documents"){
            ShowDocuments();
        }
    }

    if (words.size() == 2){
        string word0 = words[0];
        string word1 = words[1];
        if (word0 == "sign" && word1 == "out"){
            SignOut();
        }
        else if (word0 == "collection" && word1 == "out"){
            CollectionOut();
        }
        else if (word0 == "create" && word1 == "collection"){
            CreateCollection();
        }
        else if (word0 == "create" && word1 == "document"){
            CreateDocument();
        }
        else if (word0 == "document" && word1 == "random") {
            ShowRandomDocument();
        }
        else if (word0 == "document") {
            ShowDocument(word1);
        }

    }
    if (words.size() == 3){
        string word0 = words[0];
        string word1 = words[1];
        string word2 = words[2];
        if (word0 == "collection" && word1 == "in") {
            WorkWithCollection(word2);
        }
    }

    if (words.size() == 4){
        string word0 = words[0];
        string word1 = words[1];
        string word2 = words[2];
        string word3 = words[3];
        if (word0 == "sign" && word1 == "in") {
            SignIn(word2, word3);
        }
        else if (word0 == "edit" && word1 == "collection" && word2 == "description") {
            EditCollectionDescription(word3);
        }
        else if (word0 == "edit" && word1 == "collection" && word2 == "action") {
            EditCollectionAction(word3);
        }
        else if (word0 == "remove" && word1 == "collection" && word2 == "action") {
            RemoveCollectionAction(word3);
        }
        else if (word0 == "add" && word1 == "document" && word2 == "field") {
            AddDocumentField(word3);
        }
        else if (word0 == "remove" && word1 == "document" && word2 == "field") {
            RemoveDocumentField(word3);
        }
    }

    if (words.size() == 5) {
        string word0 = words[0];
        string word1 = words[1];
        string word2 = words[2];
        string word3 = words[3];
        string word4 = words[4];
        if (word0 == "sign" && word1 == "up") {
            SignUp(word2, word3, word4);
        }
        else if (word0 == "edit" && word1 == "document" && word2 == "field"){
            EditDocumentField(word3, word4);
        }
    }

    return 0;
}